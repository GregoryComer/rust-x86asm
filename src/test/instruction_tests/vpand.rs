use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpand_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 219, 227], OperandSize::Dword)
}

fn vpand_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 219, 52, 138], OperandSize::Dword)
}

fn vpand_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 219, 230], OperandSize::Qword)
}

fn vpand_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RCX, 825487817, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 219, 145, 201, 241, 51, 49], OperandSize::Qword)
}

fn vpand_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 219, 246], OperandSize::Dword)
}

fn vpand_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 219, 35], OperandSize::Dword)
}

fn vpand_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 219, 192], OperandSize::Qword)
}

fn vpand_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1802203169, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 219, 12, 245, 33, 112, 107, 107], OperandSize::Qword)
}

