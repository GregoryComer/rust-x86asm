use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvttsd2usi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 127, 24, 120, 208], OperandSize::Dword)
}

fn vcvttsd2usi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Eight, 186095352, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 120, 140, 217, 248, 150, 23, 11], OperandSize::Dword)
}

fn vcvttsd2usi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 209, 127, 24, 120, 201], OperandSize::Qword)
}

fn vcvttsd2usi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 120, 60, 208], OperandSize::Qword)
}

fn vcvttsd2usi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(RBP)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 255, 24, 120, 235], OperandSize::Qword)
}

fn vcvttsd2usi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSD2USI, operand1: Some(Direct(RCX)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 8, 120, 14], OperandSize::Qword)
}

