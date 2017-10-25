use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsignw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 9, 192], OperandSize::Dword)
}

fn vpsignw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 9, 30], OperandSize::Dword)
}

fn vpsignw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 9, 254], OperandSize::Qword)
}

fn vpsignw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 9, 35], OperandSize::Qword)
}

fn vpsignw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 9, 198], OperandSize::Dword)
}

fn vpsignw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 455920594, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 9, 148, 135, 210, 203, 44, 27], OperandSize::Dword)
}

fn vpsignw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 9, 225], OperandSize::Qword)
}

fn vpsignw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 998457391, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 9, 52, 245, 47, 64, 131, 59], OperandSize::Qword)
}

