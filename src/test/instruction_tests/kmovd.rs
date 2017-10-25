use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kmovd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K3)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 144, 222], OperandSize::Dword)
}

fn kmovd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K2)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 1093590715, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 144, 148, 78, 187, 222, 46, 65], OperandSize::Dword)
}

fn kmovd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K4)), operand2: Some(Direct(K1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 144, 225], OperandSize::Qword)
}

fn kmovd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K2)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 1486076896, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 144, 20, 197, 224, 187, 147, 88], OperandSize::Qword)
}

fn kmovd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 145, 44, 88], OperandSize::Dword)
}

fn kmovd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 1677671890, Some(OperandSize::Dword), None)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 145, 148, 187, 210, 61, 255, 99], OperandSize::Qword)
}

fn kmovd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K3)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 146, 222], OperandSize::Dword)
}

fn kmovd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K7)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 146, 255], OperandSize::Qword)
}

fn kmovd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(ESI)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 147, 246], OperandSize::Dword)
}

fn kmovd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(EDI)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 147, 255], OperandSize::Qword)
}

