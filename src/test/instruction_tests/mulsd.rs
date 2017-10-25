use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn mulsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 89, 206], OperandSize::Dword)
}

fn mulsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EAX, 986106640, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 89, 136, 16, 203, 198, 58], OperandSize::Dword)
}

fn mulsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 89, 224], OperandSize::Qword)
}

fn mulsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1429936366, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 89, 12, 245, 238, 24, 59, 85], OperandSize::Qword)
}

