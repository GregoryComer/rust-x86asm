use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn mulx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBX)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 99, 246, 211], OperandSize::Dword)
}

fn mulx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBP)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1059438892, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 83, 246, 12, 213, 44, 193, 37, 63], OperandSize::Dword)
}

fn mulx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESP)), operand3: Some(Direct(ESI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 91, 246, 206], OperandSize::Qword)
}

fn mulx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(EBX)), operand2: Some(Direct(ECX)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 1205253571, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 115, 246, 156, 73, 195, 181, 214, 71], OperandSize::Qword)
}

fn mulx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(RBX)), operand2: Some(Direct(RSI)), operand3: Some(Direct(RCX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 203, 246, 217], OperandSize::Qword)
}

fn mulx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(RSI)), operand2: Some(Direct(RDI)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 195, 246, 51], OperandSize::Qword)
}

