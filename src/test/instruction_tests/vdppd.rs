use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vdppd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(49)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 65, 223, 49], OperandSize::Dword)
}

fn vdppd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(105)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 65, 28, 65, 105], OperandSize::Dword)
}

fn vdppd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 65, 229, 116], OperandSize::Qword)
}

fn vdppd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(66)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 65, 28, 217, 66], OperandSize::Qword)
}

