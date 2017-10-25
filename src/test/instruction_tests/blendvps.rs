use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn blendvps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 20, 251], OperandSize::Dword)
}

fn blendvps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 20, 28, 193], OperandSize::Dword)
}

fn blendvps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 20, 196], OperandSize::Qword)
}

fn blendvps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 449962341, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 20, 52, 253, 101, 225, 209, 26], OperandSize::Qword)
}

