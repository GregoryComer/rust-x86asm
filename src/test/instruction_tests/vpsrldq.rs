use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsrldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 115, 216, 19], OperandSize::Dword)
}

fn vpsrldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 115, 216, 72], OperandSize::Qword)
}

fn vpsrldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 115, 220, 83], OperandSize::Dword)
}

fn vpsrldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 115, 219, 62], OperandSize::Qword)
}

fn vpsrldq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 115, 216, 111], OperandSize::Dword)
}

fn vpsrldq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1171680613, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 117, 8, 115, 28, 133, 101, 109, 214, 69, 103], OperandSize::Dword)
}

fn vpsrldq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM19)), operand3: Some(Literal8(64)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 177, 85, 8, 115, 219, 64], OperandSize::Qword)
}

fn vpsrldq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(XMM23)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 69, 0, 115, 28, 72, 125], OperandSize::Qword)
}

fn vpsrldq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 115, 222, 1], OperandSize::Dword)
}

fn vpsrldq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 109, 40, 115, 28, 83, 34], OperandSize::Dword)
}

fn vpsrldq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM21)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 177, 109, 32, 115, 221, 39], OperandSize::Qword)
}

fn vpsrldq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(YMM31)), operand2: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 5, 32, 115, 26, 109], OperandSize::Qword)
}

fn vpsrldq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 85, 72, 115, 219, 36], OperandSize::Dword)
}

fn vpsrldq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 1559134414, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 69, 72, 115, 156, 178, 206, 128, 238, 92, 36], OperandSize::Dword)
}

fn vpsrldq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM28)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 145, 61, 64, 115, 220, 102], OperandSize::Qword)
}

fn vpsrldq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLDQ, operand1: Some(Direct(ZMM14)), operand2: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 13, 72, 115, 31, 90], OperandSize::Qword)
}

