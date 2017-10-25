use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpcmpestri_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRI, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 97, 234, 77], OperandSize::Dword)
}

fn vpcmpestri_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRI, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1433512789, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 97, 4, 77, 85, 171, 113, 85, 28], OperandSize::Dword)
}

fn vpcmpestri_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRI, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 97, 196, 74], OperandSize::Qword)
}

fn vpcmpestri_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPESTRI, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 2041908509, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(52)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 97, 164, 147, 29, 13, 181, 121, 52], OperandSize::Qword)
}

