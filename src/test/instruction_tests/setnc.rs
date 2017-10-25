use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn setnc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 193], OperandSize::Word)
}

fn setnc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 183, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 128, 183, 0], OperandSize::Word)
}

fn setnc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 193], OperandSize::Dword)
}

fn setnc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(IndirectScaledDisplaced(ESI, Two, 1562267705, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 4, 117, 57, 80, 30, 93], OperandSize::Dword)
}

fn setnc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 193], OperandSize::Qword)
}

fn setnc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 1380539142, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 132, 254, 6, 91, 73, 82], OperandSize::Qword)
}

fn setnc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 193], OperandSize::Qword)
}

fn setnc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 953138926, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 132, 129, 238, 190, 207, 56], OperandSize::Qword)
}

