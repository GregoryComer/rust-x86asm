use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpcmpestrm_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRM, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 96, 250, 112], OperandSize::Dword)
}

fn vpcmpestrm_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRM, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EDX, 126326867, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 96, 154, 83, 152, 135, 7, 93], OperandSize::Dword)
}

fn vpcmpestrm_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRM, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 96, 211, 16], OperandSize::Qword)
}

fn vpcmpestrm_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRM, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 818232471, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 96, 20, 117, 151, 60, 197, 48, 38], OperandSize::Qword)
}

