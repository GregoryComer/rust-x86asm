use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpabsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 28, 227], OperandSize::Dword)
}

#[test]
fn vpabsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1318246998, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 28, 60, 245, 86, 218, 146, 78], OperandSize::Dword)
}

#[test]
fn vpabsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 28, 251], OperandSize::Qword)
}

#[test]
fn vpabsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 28, 20, 200], OperandSize::Qword)
}

#[test]
fn vpabsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 28, 203], OperandSize::Dword)
}

#[test]
fn vpabsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 277806815, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 28, 20, 253, 223, 254, 142, 16], OperandSize::Dword)
}

#[test]
fn vpabsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 28, 240], OperandSize::Qword)
}

#[test]
fn vpabsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 28, 36, 129], OperandSize::Qword)
}

#[test]
fn vpabsb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 28, 212], OperandSize::Dword)
}

#[test]
fn vpabsb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 1689764562, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 28, 172, 144, 210, 194, 183, 100], OperandSize::Dword)
}

#[test]
fn vpabsb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 162, 125, 142, 28, 238], OperandSize::Qword)
}

#[test]
fn vpabsb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 957894823, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 28, 44, 213, 167, 80, 24, 57], OperandSize::Qword)
}

#[test]
fn vpabsb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 28, 240], OperandSize::Dword)
}

#[test]
fn vpabsb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 28, 36, 152], OperandSize::Dword)
}

#[test]
fn vpabsb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 125, 174, 28, 234], OperandSize::Qword)
}

#[test]
fn vpabsb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 28, 31], OperandSize::Qword)
}

#[test]
fn vpabsb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 28, 229], OperandSize::Dword)
}

#[test]
fn vpabsb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(ZMM2)), operand2: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 28, 16], OperandSize::Dword)
}

#[test]
fn vpabsb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 130, 125, 206, 28, 209], OperandSize::Qword)
}

#[test]
fn vpabsb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(ZMM8)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 125, 202, 28, 4, 178], OperandSize::Qword)
}

