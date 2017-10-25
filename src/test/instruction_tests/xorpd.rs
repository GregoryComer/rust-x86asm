use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn xorpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 87, 204], OperandSize::Dword)
}

fn xorpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 99871756, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 87, 36, 245, 12, 236, 243, 5], OperandSize::Dword)
}

fn xorpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 87, 240], OperandSize::Qword)
}

fn xorpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 646517548, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 87, 164, 75, 44, 19, 137, 38], OperandSize::Qword)
}

