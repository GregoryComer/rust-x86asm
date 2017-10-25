use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn shrx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESI)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 107, 247, 246], OperandSize::Dword)
}

fn shrx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 2039567293, Some(OperandSize::Dword), None)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 83, 247, 44, 205, 189, 83, 145, 121], OperandSize::Dword)
}

fn shrx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDI)), operand3: Some(Direct(ESI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 75, 247, 239], OperandSize::Qword)
}

fn shrx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(RCX, 64390691, Some(OperandSize::Dword), None)), operand3: Some(Direct(ESI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 75, 247, 145, 35, 134, 214, 3], OperandSize::Qword)
}

fn shrx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(RDI)), operand2: Some(Direct(RSI)), operand3: Some(Direct(RSP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 219, 247, 254], OperandSize::Qword)
}

fn shrx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(RCX)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: Some(Direct(RDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 195, 247, 9], OperandSize::Qword)
}

