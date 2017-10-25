use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpslldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 115, 251, 113], OperandSize::Dword)
}

fn vpslldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(106)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 115, 252, 106], OperandSize::Qword)
}

fn vpslldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 115, 253, 111], OperandSize::Dword)
}

fn vpslldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 115, 253, 105], OperandSize::Qword)
}

fn vpslldq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 115, 250, 110], OperandSize::Dword)
}

fn vpslldq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 101, 8, 115, 60, 83, 36], OperandSize::Dword)
}

fn vpslldq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM30)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 145, 61, 0, 115, 254, 60], OperandSize::Qword)
}

fn vpslldq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 77, 8, 115, 59, 17], OperandSize::Qword)
}

fn vpslldq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 115, 254, 45], OperandSize::Dword)
}

fn vpslldq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Four, 470265099, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 77, 40, 115, 188, 145, 11, 173, 7, 28, 1], OperandSize::Dword)
}

fn vpslldq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM13)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 209, 117, 32, 115, 253, 84], OperandSize::Qword)
}

fn vpslldq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 69, 40, 115, 60, 144, 65], OperandSize::Qword)
}

fn vpslldq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 85, 72, 115, 248, 14], OperandSize::Dword)
}

fn vpslldq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 1414423228, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 85, 72, 115, 188, 119, 188, 98, 78, 84, 103], OperandSize::Dword)
}

fn vpslldq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 72, 115, 250, 94], OperandSize::Qword)
}

fn vpslldq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLDQ, operand1: Some(Direct(ZMM30)), operand2: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 13, 64, 115, 62, 100], OperandSize::Qword)
}

