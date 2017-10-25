use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsignd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 10, 198], OperandSize::Dword)
}

fn vpsignd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 10, 33], OperandSize::Dword)
}

fn vpsignd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 10, 245], OperandSize::Qword)
}

fn vpsignd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 10, 16], OperandSize::Qword)
}

fn vpsignd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 10, 227], OperandSize::Dword)
}

fn vpsignd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1903380718, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 10, 36, 221, 238, 72, 115, 113], OperandSize::Dword)
}

fn vpsignd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 10, 253], OperandSize::Qword)
}

fn vpsignd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 10, 20, 179], OperandSize::Qword)
}

