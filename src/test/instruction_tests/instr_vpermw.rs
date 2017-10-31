use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 138, 141, 254], OperandSize::Dword)
}

#[test]
fn vpermw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 138, 141, 4, 255], OperandSize::Dword)
}

#[test]
fn vpermw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 130, 245, 142, 141, 246], OperandSize::Qword)
}

#[test]
fn vpermw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectDisplaced(RAX, 888843811, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 221, 130, 141, 136, 35, 174, 250, 52], OperandSize::Qword)
}

#[test]
fn vpermw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 174, 141, 231], OperandSize::Dword)
}

#[test]
fn vpermw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 1120894350, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 171, 141, 148, 206, 142, 125, 207, 66], OperandSize::Dword)
}

#[test]
fn vpermw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 130, 133, 172, 141, 253], OperandSize::Qword)
}

#[test]
fn vpermw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Two, 508383596, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 229, 174, 141, 156, 89, 108, 81, 77, 30], OperandSize::Qword)
}

#[test]
fn vpermw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 204, 141, 192], OperandSize::Dword)
}

#[test]
fn vpermw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1546314395, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 141, 4, 117, 155, 226, 42, 92], OperandSize::Dword)
}

#[test]
fn vpermw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 213, 196, 141, 222], OperandSize::Qword)
}

#[test]
fn vpermw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 821934415, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 181, 201, 141, 60, 133, 79, 185, 253, 48], OperandSize::Qword)
}

