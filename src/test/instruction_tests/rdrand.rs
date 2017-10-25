use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn rdrand_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RDRAND, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 199, 244], OperandSize::Dword)
}

fn rdrand_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RDRAND, operand1: Some(Direct(CX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 199, 241], OperandSize::Qword)
}

fn rdrand_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RDRAND, operand1: Some(Direct(EBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 243], OperandSize::Dword)
}

fn rdrand_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RDRAND, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 241], OperandSize::Qword)
}

fn rdrand_5() {
    run_test(&Instruction { mnemonic: Mnemonic::RDRAND, operand1: Some(Direct(RCX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 199, 241], OperandSize::Qword)
}

