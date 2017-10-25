use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fisttp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 186, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 136, 186, 0], OperandSize::Word)
}

fn fisttp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledDisplaced(EDI, Four, 1398193257, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 12, 189, 105, 188, 86, 83], OperandSize::Dword)
}

fn fisttp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 1631584454, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 140, 223, 198, 0, 64, 97], OperandSize::Qword)
}

fn fisttp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectDisplaced(SI, 173, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 140, 173, 0], OperandSize::Word)
}

fn fisttp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectDisplaced(EDX, 1885299570, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 138, 114, 99, 95, 112], OperandSize::Dword)
}

fn fisttp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 11], OperandSize::Qword)
}

fn fisttp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 241, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 137, 241, 0], OperandSize::Word)
}

fn fisttp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 699196356, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 140, 146, 196, 227, 172, 41], OperandSize::Dword)
}

fn fisttp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 2077230726, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 140, 203, 134, 6, 208, 123], OperandSize::Qword)
}

