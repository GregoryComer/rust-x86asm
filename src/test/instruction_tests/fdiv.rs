use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fdiv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(IndirectDisplaced(BX, 113, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 119, 113], OperandSize::Word)
}

fn fdiv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 186187721, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 180, 177, 201, 255, 24, 11], OperandSize::Dword)
}

fn fdiv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1564988412, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 52, 213, 252, 211, 71, 93], OperandSize::Qword)
}

fn fdiv_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST)), operand2: Some(Direct(ST5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 245], OperandSize::Word)
}

fn fdiv_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST)), operand2: Some(Direct(ST4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 244], OperandSize::Dword)
}

fn fdiv_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST)), operand2: Some(Direct(ST4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 244], OperandSize::Qword)
}

fn fdiv_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(IndirectDisplaced(BP, 23669, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 182, 117, 92], OperandSize::Word)
}

fn fdiv_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Four, 1864324508, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 180, 155, 156, 85, 31, 111], OperandSize::Dword)
}

fn fdiv_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(IndirectDisplaced(RDI, 738384386, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 183, 2, 218, 2, 44], OperandSize::Qword)
}

fn fdiv_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST7)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 255], OperandSize::Word)
}

fn fdiv_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST2)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 250], OperandSize::Dword)
}

fn fdiv_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST1)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 249], OperandSize::Qword)
}

