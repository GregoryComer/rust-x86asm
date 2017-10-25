use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsignb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 8, 247], OperandSize::Dword)
}

fn vpsignb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EDX, 390080325, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 8, 154, 69, 39, 64, 23], OperandSize::Dword)
}

fn vpsignb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 8, 226], OperandSize::Qword)
}

fn vpsignb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 8, 58], OperandSize::Qword)
}

fn vpsignb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 8, 212], OperandSize::Dword)
}

fn vpsignb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 8, 2], OperandSize::Dword)
}

fn vpsignb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 8, 252], OperandSize::Qword)
}

fn vpsignb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 2098136269, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 8, 4, 213, 205, 4, 15, 125], OperandSize::Qword)
}

