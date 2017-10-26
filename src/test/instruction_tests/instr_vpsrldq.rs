use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 115, 223, 84], OperandSize::Dword)
}

#[test]
fn vpsrldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 115, 219, 97], OperandSize::Qword)
}

#[test]
fn vpsrldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 115, 218, 73], OperandSize::Dword)
}

#[test]
fn vpsrldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 115, 217, 124], OperandSize::Qword)
}

#[test]
fn vpsrldq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 115, 220, 41], OperandSize::Dword)
}

#[test]
fn vpsrldq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Two, 172503583, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 69, 8, 115, 156, 74, 31, 50, 72, 10, 95], OperandSize::Dword)
}

#[test]
fn vpsrldq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM12)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 193, 105, 115, 220, 71], OperandSize::Qword)
}

#[test]
fn vpsrldq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM26)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1771083040, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 45, 0, 115, 28, 221, 32, 149, 144, 105, 63], OperandSize::Qword)
}

#[test]
fn vpsrldq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 115, 221, 7], OperandSize::Dword)
}

#[test]
fn vpsrldq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(EAX, ESI, Two, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 77, 40, 115, 28, 112, 83], OperandSize::Dword)
}

#[test]
fn vpsrldq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 115, 217, 26], OperandSize::Qword)
}

#[test]
fn vpsrldq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 2107282494, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 69, 40, 115, 28, 69, 62, 148, 154, 125, 88], OperandSize::Qword)
}

#[test]
fn vpsrldq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 93, 72, 115, 218, 73], OperandSize::Dword)
}

#[test]
fn vpsrldq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 122558276, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 77, 72, 115, 156, 153, 68, 23, 78, 7, 33], OperandSize::Dword)
}

#[test]
fn vpsrldq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM17)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 177, 21, 72, 115, 217, 124], OperandSize::Qword)
}

#[test]
fn vpsrldq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Four, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 53, 64, 115, 28, 151, 110], OperandSize::Qword)
}

