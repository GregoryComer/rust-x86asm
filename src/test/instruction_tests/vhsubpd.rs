use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vhsubpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 125, 212], OperandSize::Dword)
}

fn vhsubpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 125, 10], OperandSize::Dword)
}

fn vhsubpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 125, 218], OperandSize::Qword)
}

fn vhsubpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 125, 38], OperandSize::Qword)
}

fn vhsubpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 125, 215], OperandSize::Dword)
}

fn vhsubpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Four, 601255595, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 125, 172, 136, 171, 110, 214, 35], OperandSize::Dword)
}

fn vhsubpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 125, 220], OperandSize::Qword)
}

fn vhsubpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 125, 44, 113], OperandSize::Qword)
}

