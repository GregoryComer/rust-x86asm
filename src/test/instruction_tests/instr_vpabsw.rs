use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpabsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 29, 242], OperandSize::Dword)
}

#[test]
fn vpabsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 29, 20, 203], OperandSize::Dword)
}

#[test]
fn vpabsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 29, 210], OperandSize::Qword)
}

#[test]
fn vpabsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 29, 4, 211], OperandSize::Qword)
}

#[test]
fn vpabsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 29, 225], OperandSize::Dword)
}

#[test]
fn vpabsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 29, 43], OperandSize::Dword)
}

#[test]
fn vpabsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 29, 238], OperandSize::Qword)
}

#[test]
fn vpabsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 29, 41], OperandSize::Qword)
}

#[test]
fn vpabsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 29, 241], OperandSize::Dword)
}

#[test]
fn vpabsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 1352361642, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 29, 148, 193, 170, 102, 155, 80], OperandSize::Dword)
}

#[test]
fn vpabsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 2, 125, 139, 29, 227], OperandSize::Qword)
}

#[test]
fn vpabsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RBX, 1492315823, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 29, 155, 175, 238, 242, 88], OperandSize::Qword)
}

