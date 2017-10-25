use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn setg_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 194], OperandSize::Word)
}

fn setg_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 21010, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 128, 18, 82], OperandSize::Word)
}

fn setg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 193], OperandSize::Dword)
}

fn setg_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Two, 312282781, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 132, 70, 157, 14, 157, 18], OperandSize::Dword)
}

fn setg_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 193], OperandSize::Qword)
}

fn setg_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 4, 246], OperandSize::Qword)
}

fn setg_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 193], OperandSize::Qword)
}

fn setg_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(IndirectScaledDisplaced(RDI, Two, 884014191, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 4, 125, 111, 252, 176, 52], OperandSize::Qword)
}

