use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp14sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 141, 77, 222], OperandSize::Dword)
}

#[test]
fn vrcp14sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 477727995, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 139, 77, 172, 112, 251, 140, 121, 28], OperandSize::Dword)
}

#[test]
fn vrcp14sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 178, 165, 139, 77, 200], OperandSize::Qword)
}

#[test]
fn vrcp14sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 173, 139, 77, 28, 74], OperandSize::Qword)
}

