use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendmw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 137, 102, 214], OperandSize::Dword)
}

#[test]
fn vpblendmw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 138, 102, 55], OperandSize::Dword)
}

#[test]
fn vpblendmw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 194, 237, 137, 102, 209], OperandSize::Qword)
}

#[test]
fn vpblendmw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM20)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 221, 131, 102, 23], OperandSize::Qword)
}

#[test]
fn vpblendmw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 172, 102, 192], OperandSize::Dword)
}

#[test]
fn vpblendmw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 113724613, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 172, 102, 12, 133, 197, 76, 199, 6], OperandSize::Dword)
}

#[test]
fn vpblendmw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 237, 175, 102, 219], OperandSize::Qword)
}

#[test]
fn vpblendmw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 1077787823, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 166, 102, 140, 194, 175, 188, 61, 64], OperandSize::Qword)
}

#[test]
fn vpblendmw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 205, 102, 198], OperandSize::Dword)
}

#[test]
fn vpblendmw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 203, 102, 35], OperandSize::Dword)
}

#[test]
fn vpblendmw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM15)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 133, 201, 102, 246], OperandSize::Qword)
}

#[test]
fn vpblendmw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM8)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 189, 206, 102, 59], OperandSize::Qword)
}

