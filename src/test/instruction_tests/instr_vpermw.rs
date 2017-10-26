use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 137, 141, 231], OperandSize::Dword)
}

#[test]
fn vpermw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 137, 141, 36, 201], OperandSize::Dword)
}

#[test]
fn vpermw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 165, 132, 141, 232], OperandSize::Qword)
}

#[test]
fn vpermw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 189, 138, 141, 52, 218], OperandSize::Qword)
}

#[test]
fn vpermw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 172, 141, 198], OperandSize::Dword)
}

#[test]
fn vpermw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 2127472702, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 175, 141, 60, 197, 62, 168, 206, 126], OperandSize::Dword)
}

#[test]
fn vpermw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 162, 181, 174, 141, 203], OperandSize::Qword)
}

#[test]
fn vpermw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectDisplaced(RDX, 1323108919, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 165, 174, 141, 178, 55, 10, 221, 78], OperandSize::Qword)
}

#[test]
fn vpermw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 141, 235], OperandSize::Dword)
}

#[test]
fn vpermw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 1757738953, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 202, 141, 172, 191, 201, 247, 196, 104], OperandSize::Dword)
}

#[test]
fn vpermw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 157, 203, 141, 201], OperandSize::Qword)
}

#[test]
fn vpermw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 758794272, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 189, 203, 141, 36, 213, 32, 72, 58, 45], OperandSize::Qword)
}

