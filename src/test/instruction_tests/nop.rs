use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn nop_1() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 196], OperandSize::Word)
}

fn nop_2() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Indirect(BX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 7], OperandSize::Word)
}

fn nop_3() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 199], OperandSize::Dword)
}

fn nop_4() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Indirect(EDI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 7], OperandSize::Dword)
}

fn nop_5() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 194], OperandSize::Qword)
}

fn nop_6() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Indirect(RBX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 3], OperandSize::Qword)
}

fn nop_7() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 193], OperandSize::Word)
}

fn nop_8() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 4411, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 31, 131, 59, 17], OperandSize::Word)
}

fn nop_9() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(EBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 197], OperandSize::Dword)
}

fn nop_10() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 7], OperandSize::Dword)
}

fn nop_11() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 193], OperandSize::Qword)
}

fn nop_12() {
    run_test(&Instruction { mnemonic: Mnemonic::NOP, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 1935230084, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 31, 132, 113, 132, 68, 89, 115], OperandSize::Qword)
}

