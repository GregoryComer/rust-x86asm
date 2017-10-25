use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fadd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 93, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 64, 93], OperandSize::Word)
}

fn fadd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 1], OperandSize::Dword)
}

fn fadd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(IndirectScaledDisplaced(RAX, Two, 965965702, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 4, 69, 134, 119, 147, 57], OperandSize::Qword)
}

fn fadd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST)), operand2: Some(Direct(ST1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 193], OperandSize::Word)
}

fn fadd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST)), operand2: Some(Direct(ST4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 196], OperandSize::Dword)
}

fn fadd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST)), operand2: Some(Direct(ST7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 199], OperandSize::Qword)
}

fn fadd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(IndirectDisplaced(BX, 25371, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 135, 27, 99], OperandSize::Word)
}

fn fadd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(IndirectDisplaced(EDI, 719950829, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 135, 237, 147, 233, 42], OperandSize::Dword)
}

fn fadd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 2099625154, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 4, 197, 194, 188, 37, 125], OperandSize::Qword)
}

fn fadd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST5)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 197], OperandSize::Word)
}

fn fadd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST3)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 195], OperandSize::Dword)
}

fn fadd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST3)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 195], OperandSize::Qword)
}

