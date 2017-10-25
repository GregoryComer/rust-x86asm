use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvttpd2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 230, 216], OperandSize::Dword)
}

fn vcvttpd2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 230, 56], OperandSize::Dword)
}

fn vcvttpd2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 230, 253], OperandSize::Qword)
}

fn vcvttpd2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 230, 20, 222], OperandSize::Qword)
}

fn vcvttpd2dq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 230, 196], OperandSize::Dword)
}

fn vcvttpd2dq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 1838528519, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 230, 20, 213, 7, 184, 149, 109], OperandSize::Dword)
}

fn vcvttpd2dq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 230, 248], OperandSize::Qword)
}

fn vcvttpd2dq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 230, 10], OperandSize::Qword)
}

fn vcvttpd2dq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 142, 230, 237], OperandSize::Dword)
}

fn vcvttpd2dq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Eight, 1714394900, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 142, 230, 156, 200, 20, 151, 47, 102], OperandSize::Dword)
}

fn vcvttpd2dq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 33, 253, 141, 230, 212], OperandSize::Qword)
}

fn vcvttpd2dq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM23)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1206488146, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 253, 138, 230, 60, 93, 82, 140, 233, 71], OperandSize::Qword)
}

fn vcvttpd2dq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 169, 230, 211], OperandSize::Dword)
}

fn vcvttpd2dq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Four, 950694820, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 170, 230, 156, 136, 164, 115, 170, 56], OperandSize::Dword)
}

fn vcvttpd2dq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM20)), operand2: Some(Direct(YMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 129, 253, 169, 230, 229], OperandSize::Qword)
}

fn vcvttpd2dq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM9)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 253, 174, 230, 12, 241], OperandSize::Qword)
}

fn vcvttpd2dq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 155, 230, 197], OperandSize::Dword)
}

fn vcvttpd2dq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Two, 185648563, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 202, 230, 148, 88, 179, 197, 16, 11], OperandSize::Dword)
}

fn vcvttpd2dq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(YMM13)), operand2: Some(Direct(ZMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 81, 253, 157, 230, 234], OperandSize::Qword)
}

fn vcvttpd2dq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1211024401, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 206, 230, 28, 189, 17, 196, 46, 72], OperandSize::Qword)
}

