use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovhpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 22, 27], OperandSize::Dword)
}

fn vmovhpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RDI, 917894432, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 22, 191, 32, 245, 181, 54], OperandSize::Qword)
}

fn vmovhpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 22, 36, 218], OperandSize::Dword)
}

fn vmovhpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectDisplaced(RDX, 1153023902, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 1, 22, 162, 158, 191, 185, 68], OperandSize::Qword)
}

fn vmovhpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 23, 24], OperandSize::Dword)
}

fn vmovhpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 617568289, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 23, 188, 250, 33, 88, 207, 36], OperandSize::Qword)
}

fn vmovhpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 130522236, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 23, 172, 130, 124, 156, 199, 7], OperandSize::Dword)
}

fn vmovhpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(IndirectScaledDisplaced(RCX, Two, 1923087572, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 121, 23, 36, 77, 212, 252, 159, 114], OperandSize::Qword)
}

