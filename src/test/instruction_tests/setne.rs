use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn setne_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 193], OperandSize::Word)
}

fn setne_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 2725, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 131, 165, 10], OperandSize::Word)
}

fn setne_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 195], OperandSize::Dword)
}

fn setne_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(IndirectDisplaced(ECX, 1041130851, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 129, 99, 101, 14, 62], OperandSize::Dword)
}

fn setne_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 193], OperandSize::Qword)
}

fn setne_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1630667080, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 4, 213, 72, 1, 50, 97], OperandSize::Qword)
}

fn setne_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 193], OperandSize::Qword)
}

fn setne_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNE, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 1730729533, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 149, 132, 177, 61, 214, 40, 103], OperandSize::Qword)
}

