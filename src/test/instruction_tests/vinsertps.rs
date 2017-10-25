use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vinsertps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 33, 228, 92], OperandSize::Dword)
}

fn vinsertps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(48)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 33, 52, 66, 48], OperandSize::Dword)
}

fn vinsertps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(44)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 33, 213, 44], OperandSize::Qword)
}

fn vinsertps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Dword), None)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 33, 52, 153, 8], OperandSize::Qword)
}

fn vinsertps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 33, 226, 6], OperandSize::Dword)
}

fn vinsertps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Eight, 616657146, Some(OperandSize::Dword), None)), operand4: Some(Literal8(28)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 33, 188, 242, 250, 112, 193, 36, 28], OperandSize::Dword)
}

fn vinsertps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(28)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 99, 69, 0, 33, 254, 28], OperandSize::Qword)
}

fn vinsertps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 227, 29, 8, 33, 12, 94, 70], OperandSize::Qword)
}

