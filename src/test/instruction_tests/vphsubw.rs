use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vphsubw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 5, 216], OperandSize::Dword)
}

fn vphsubw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ECX, 2053526348, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 5, 153, 76, 83, 102, 122], OperandSize::Dword)
}

fn vphsubw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 5, 228], OperandSize::Qword)
}

fn vphsubw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 588575200, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 5, 164, 79, 224, 241, 20, 35], OperandSize::Qword)
}

fn vphsubw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 5, 230], OperandSize::Dword)
}

fn vphsubw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EBX, 305795732, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 5, 179, 148, 18, 58, 18], OperandSize::Dword)
}

fn vphsubw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 5, 200], OperandSize::Qword)
}

fn vphsubw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 5, 44, 198], OperandSize::Qword)
}

