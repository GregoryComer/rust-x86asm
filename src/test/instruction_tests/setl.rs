use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn setl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 193], OperandSize::Word)
}

fn setl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(IndirectDisplaced(DI, 228, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 133, 228, 0], OperandSize::Word)
}

fn setl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 193], OperandSize::Dword)
}

fn setl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(IndirectDisplaced(ECX, 1410867343, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 129, 143, 32, 24, 84], OperandSize::Dword)
}

fn setl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 193], OperandSize::Qword)
}

fn setl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(IndirectDisplaced(RCX, 512055242, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 129, 202, 87, 133, 30], OperandSize::Qword)
}

fn setl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 194], OperandSize::Qword)
}

fn setl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(IndirectDisplaced(RDI, 1234384609, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 135, 225, 54, 147, 73], OperandSize::Qword)
}

