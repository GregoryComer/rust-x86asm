use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vroundss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 10, 236, 114], OperandSize::Dword)
}

fn vroundss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(85)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 10, 4, 241, 85], OperandSize::Dword)
}

fn vroundss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 10, 218, 1], OperandSize::Qword)
}

fn vroundss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Dword), None)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 10, 52, 138, 59], OperandSize::Qword)
}

