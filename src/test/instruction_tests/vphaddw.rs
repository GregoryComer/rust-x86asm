use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vphaddw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 1, 218], OperandSize::Dword)
}

fn vphaddw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EDX, 687867673, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 1, 178, 25, 7, 0, 41], OperandSize::Dword)
}

fn vphaddw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 1, 221], OperandSize::Qword)
}

fn vphaddw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RSI, 1691402781, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 1, 134, 29, 194, 208, 100], OperandSize::Qword)
}

fn vphaddw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 1, 206], OperandSize::Dword)
}

fn vphaddw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 1352425034, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 1, 156, 206, 74, 94, 156, 80], OperandSize::Dword)
}

fn vphaddw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 1, 217], OperandSize::Qword)
}

fn vphaddw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1802461658, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 1, 36, 93, 218, 97, 111, 107], OperandSize::Qword)
}

