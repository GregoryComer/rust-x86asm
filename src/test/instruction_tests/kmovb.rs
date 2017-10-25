use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kmovb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K5)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 144, 238], OperandSize::Dword)
}

fn kmovb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K2)), operand2: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 144, 20, 217], OperandSize::Dword)
}

fn kmovb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K5)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 144, 238], OperandSize::Qword)
}

fn kmovb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K5)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 2013084484, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 144, 172, 95, 68, 59, 253, 119], OperandSize::Qword)
}

fn kmovb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Indirect(EAX, Some(OperandSize::Byte), None)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 145, 16], OperandSize::Dword)
}

fn kmovb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(IndirectDisplaced(RCX, 1769620915, Some(OperandSize::Byte), None)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 145, 185, 179, 69, 122, 105], OperandSize::Qword)
}

fn kmovb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K5)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 146, 239], OperandSize::Dword)
}

fn kmovb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(K7)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 146, 251], OperandSize::Qword)
}

fn kmovb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(EBX)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 147, 220], OperandSize::Dword)
}

fn kmovb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVB, operand1: Some(Direct(EDI)), operand2: Some(Direct(K1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 147, 249], OperandSize::Qword)
}

