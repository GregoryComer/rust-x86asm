use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn adcx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 246, 230], OperandSize::Dword)
}

fn adcx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 493954013, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 246, 60, 93, 221, 35, 113, 29], OperandSize::Dword)
}

fn adcx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 246, 228], OperandSize::Qword)
}

fn adcx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(ECX)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 246, 10], OperandSize::Qword)
}

fn adcx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(RDI)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 72, 15, 56, 246, 252], OperandSize::Qword)
}

fn adcx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 72, 15, 56, 246, 52, 88], OperandSize::Qword)
}

