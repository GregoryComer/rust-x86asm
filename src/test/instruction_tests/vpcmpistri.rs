use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpcmpistri_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRI, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 99, 214, 114], OperandSize::Dword)
}

fn vpcmpistri_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRI, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 99, 7, 41], OperandSize::Dword)
}

fn vpcmpistri_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRI, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 99, 240, 0], OperandSize::Qword)
}

fn vpcmpistri_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPISTRI, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1057295119, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 99, 20, 221, 15, 11, 5, 63, 114], OperandSize::Qword)
}

