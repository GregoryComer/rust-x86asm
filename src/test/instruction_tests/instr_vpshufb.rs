use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpshufb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 0, 238], OperandSize::Dword)
}

#[test]
fn vpshufb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 0, 20, 139], OperandSize::Dword)
}

#[test]
fn vpshufb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 0, 238], OperandSize::Qword)
}

#[test]
fn vpshufb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 575456627, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 0, 28, 181, 115, 197, 76, 34], OperandSize::Qword)
}

#[test]
fn vpshufb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 0, 227], OperandSize::Dword)
}

#[test]
fn vpshufb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 0, 47], OperandSize::Dword)
}

#[test]
fn vpshufb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 0, 239], OperandSize::Qword)
}

#[test]
fn vpshufb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 369815991, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 0, 132, 90, 183, 241, 10, 22], OperandSize::Qword)
}

#[test]
fn vpshufb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 142, 0, 251], OperandSize::Dword)
}

#[test]
fn vpshufb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ECX, 1717912244, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 117, 138, 0, 153, 180, 66, 101, 102], OperandSize::Dword)
}

#[test]
fn vpshufb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 85, 129, 0, 218], OperandSize::Qword)
}

#[test]
fn vpshufb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RDI, 890831476, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 117, 137, 0, 175, 116, 2, 25, 53], OperandSize::Qword)
}

#[test]
fn vpshufb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 172, 0, 227], OperandSize::Dword)
}

#[test]
fn vpshufb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 93, 172, 0, 46], OperandSize::Dword)
}

#[test]
fn vpshufb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM23)), operand3: Some(Direct(YMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 66, 69, 163, 0, 204], OperandSize::Qword)
}

#[test]
fn vpshufb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectDisplaced(RDX, 43134064, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 13, 162, 0, 130, 112, 44, 146, 2], OperandSize::Qword)
}

#[test]
fn vpshufb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 206, 0, 250], OperandSize::Dword)
}

#[test]
fn vpshufb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 201, 0, 52, 177], OperandSize::Dword)
}

#[test]
fn vpshufb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 34, 109, 205, 0, 238], OperandSize::Qword)
}

#[test]
fn vpshufb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM23)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 69, 196, 0, 40], OperandSize::Qword)
}

