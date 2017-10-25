use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vtestpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 15, 246], OperandSize::Dword)
}

fn vtestpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 15, 44, 147], OperandSize::Dword)
}

fn vtestpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 15, 210], OperandSize::Qword)
}

fn vtestpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 15, 20, 210], OperandSize::Qword)
}

fn vtestpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 15, 239], OperandSize::Dword)
}

fn vtestpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1773045948, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 15, 44, 157, 188, 136, 174, 105], OperandSize::Dword)
}

fn vtestpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 15, 241], OperandSize::Qword)
}

fn vtestpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPD, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 15, 49], OperandSize::Qword)
}

