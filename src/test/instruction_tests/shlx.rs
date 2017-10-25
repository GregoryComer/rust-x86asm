use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn shlx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDX)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 247, 250], OperandSize::Dword)
}

fn shlx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 604914060, Some(OperandSize::Dword), None)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 247, 164, 215, 140, 65, 14, 36], OperandSize::Dword)
}

fn shlx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESP)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 247, 228], OperandSize::Qword)
}

fn shlx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Dword), None)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 247, 28, 222], OperandSize::Qword)
}

fn shlx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(RSI)), operand2: Some(Direct(RDX)), operand3: Some(Direct(RCX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 247, 242], OperandSize::Qword)
}

fn shlx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLX, operand1: Some(Direct(RSI)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: Some(Direct(RBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 247, 51], OperandSize::Qword)
}

