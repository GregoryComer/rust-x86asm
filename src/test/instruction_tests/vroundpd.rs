use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vroundpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 9, 251, 102], OperandSize::Dword)
}

fn vroundpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 1948736479, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 9, 180, 208, 223, 91, 39, 116, 60], OperandSize::Dword)
}

fn vroundpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 9, 248, 5], OperandSize::Qword)
}

fn vroundpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 9, 62, 114], OperandSize::Qword)
}

fn vroundpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 9, 204, 75], OperandSize::Dword)
}

fn vroundpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 994189207, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 9, 180, 152, 151, 31, 66, 59, 45], OperandSize::Dword)
}

fn vroundpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 9, 197, 38], OperandSize::Qword)
}

fn vroundpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPD, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 9, 57, 113], OperandSize::Qword)
}

