use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fsub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Memory(3410, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 38, 82, 13], OperandSize::Word)
}

fn fsub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(IndirectScaledDisplaced(EDI, Eight, 481978824, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 36, 253, 200, 105, 186, 28], OperandSize::Dword)
}

fn fsub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(IndirectScaledDisplaced(RBX, Two, 583727777, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 36, 93, 161, 250, 202, 34], OperandSize::Qword)
}

fn fsub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST)), operand2: Some(Direct(ST7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 231], OperandSize::Word)
}

fn fsub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST)), operand2: Some(Direct(ST6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 230], OperandSize::Dword)
}

fn fsub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST)), operand2: Some(Direct(ST5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 229], OperandSize::Qword)
}

fn fsub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(IndirectDisplaced(SI, 27, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 100, 27], OperandSize::Word)
}

fn fsub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 54119636, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 164, 151, 212, 204, 57, 3], OperandSize::Dword)
}

fn fsub_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 33], OperandSize::Qword)
}

fn fsub_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST3)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 235], OperandSize::Word)
}

fn fsub_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST6)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 238], OperandSize::Dword)
}

fn fsub_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST4)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 236], OperandSize::Qword)
}

