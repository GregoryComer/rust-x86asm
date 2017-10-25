use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn sarx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 82, 247, 218], OperandSize::Dword)
}

fn sarx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 1176270140, Some(OperandSize::Dword), None)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 114, 247, 12, 117, 60, 117, 28, 70], OperandSize::Dword)
}

fn sarx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBP)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 114, 247, 237], OperandSize::Qword)
}

fn sarx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 401620425, Some(OperandSize::Dword), None)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 90, 247, 28, 69, 201, 61, 240, 23], OperandSize::Qword)
}

fn sarx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(RDX)), operand2: Some(Direct(RSI)), operand3: Some(Direct(RCX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 242, 247, 214], OperandSize::Qword)
}

fn sarx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Qword), None)), operand3: Some(Direct(RDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 234, 247, 12, 87], OperandSize::Qword)
}

