use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vaesenclast_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENCLAST, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 221, 240], OperandSize::Dword)
}

fn vaesenclast_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENCLAST, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EBX, 153414111, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 221, 155, 223, 233, 36, 9], OperandSize::Dword)
}

fn vaesenclast_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENCLAST, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 221, 231], OperandSize::Qword)
}

fn vaesenclast_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENCLAST, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 383278263, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 221, 20, 93, 183, 92, 216, 22], OperandSize::Qword)
}

