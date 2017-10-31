use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovhpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 22, 31], OperandSize::Dword)
}

#[test]
fn vmovhpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 226161298, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 22, 28, 149, 146, 242, 122, 13], OperandSize::Qword)
}

#[test]
fn vmovhpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EAX, 357929722, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 22, 184, 250, 146, 85, 21], OperandSize::Dword)
}

#[test]
fn vmovhpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 221, 8, 22, 12, 155], OperandSize::Qword)
}

#[test]
fn vmovhpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 23, 4, 177], OperandSize::Dword)
}

#[test]
fn vmovhpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 23, 20, 95], OperandSize::Qword)
}

#[test]
fn vmovhpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(IndirectDisplaced(ECX, 1666942009, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 23, 177, 57, 132, 91, 99], OperandSize::Dword)
}

#[test]
fn vmovhpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(IndirectDisplaced(RCX, 1465182060, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 253, 8, 23, 129, 108, 231, 84, 87], OperandSize::Qword)
}

