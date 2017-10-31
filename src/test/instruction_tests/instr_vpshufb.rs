use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpshufb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 0, 231], OperandSize::Dword)
}

#[test]
fn vpshufb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 0, 15], OperandSize::Dword)
}

#[test]
fn vpshufb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 0, 202], OperandSize::Qword)
}

#[test]
fn vpshufb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 0, 28, 194], OperandSize::Qword)
}

#[test]
fn vpshufb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 0, 228], OperandSize::Dword)
}

#[test]
fn vpshufb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 1882915833, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 0, 164, 249, 249, 3, 59, 112], OperandSize::Dword)
}

#[test]
fn vpshufb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 0, 252], OperandSize::Qword)
}

#[test]
fn vpshufb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 0, 54], OperandSize::Qword)
}

#[test]
fn vpshufb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 138, 0, 208], OperandSize::Dword)
}

#[test]
fn vpshufb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Four, 2134754715, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 101, 137, 0, 132, 184, 155, 197, 61, 127], OperandSize::Dword)
}

#[test]
fn vpshufb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 82, 85, 131, 0, 223], OperandSize::Qword)
}

#[test]
fn vpshufb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM11)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 37, 142, 0, 43], OperandSize::Qword)
}

#[test]
fn vpshufb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 171, 0, 203], OperandSize::Dword)
}

#[test]
fn vpshufb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 173, 0, 42], OperandSize::Dword)
}

#[test]
fn vpshufb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM18)), operand3: Some(Direct(YMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 109, 162, 0, 249], OperandSize::Qword)
}

#[test]
fn vpshufb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 53, 170, 0, 44, 187], OperandSize::Qword)
}

#[test]
fn vpshufb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 206, 0, 200], OperandSize::Dword)
}

#[test]
fn vpshufb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 886747034, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 202, 0, 20, 157, 154, 175, 218, 52], OperandSize::Dword)
}

#[test]
fn vpshufb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 146, 53, 205, 0, 246], OperandSize::Qword)
}

#[test]
fn vpshufb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 125287511, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 93, 193, 0, 20, 245, 87, 188, 119, 7], OperandSize::Qword)
}

