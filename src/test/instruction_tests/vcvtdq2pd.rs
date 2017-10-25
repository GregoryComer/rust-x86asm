use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtdq2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 230, 208], OperandSize::Dword)
}

fn vcvtdq2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EDI, 1300975999, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 230, 175, 127, 81, 139, 77], OperandSize::Dword)
}

fn vcvtdq2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 230, 193], OperandSize::Qword)
}

fn vcvtdq2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 230, 36, 146], OperandSize::Qword)
}

fn vcvtdq2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 230, 245], OperandSize::Dword)
}

fn vcvtdq2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 1065030866, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 230, 188, 126, 210, 20, 123, 63], OperandSize::Dword)
}

fn vcvtdq2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 230, 237], OperandSize::Qword)
}

fn vcvtdq2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 1941155998, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 230, 4, 181, 158, 176, 179, 115], OperandSize::Qword)
}

fn vcvtdq2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 140, 230, 234], OperandSize::Dword)
}

fn vcvtdq2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1825077921, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 143, 230, 28, 69, 161, 122, 200, 108], OperandSize::Dword)
}

fn vcvtdq2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 126, 142, 230, 243], OperandSize::Qword)
}

fn vcvtdq2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM24)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 126, 138, 230, 2], OperandSize::Qword)
}

fn vcvtdq2pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 169, 230, 236], OperandSize::Dword)
}

fn vcvtdq2pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 174, 230, 4, 81], OperandSize::Dword)
}

fn vcvtdq2pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(XMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 17, 126, 174, 230, 193], OperandSize::Qword)
}

fn vcvtdq2pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM17)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 237022435, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 126, 170, 230, 12, 245, 227, 172, 32, 14], OperandSize::Qword)
}

fn vcvtdq2pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 203, 230, 214], OperandSize::Dword)
}

fn vcvtdq2pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 1548776071, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 201, 230, 180, 183, 135, 114, 80, 92], OperandSize::Dword)
}

fn vcvtdq2pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(YMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 33, 126, 203, 230, 223], OperandSize::Qword)
}

fn vcvtdq2pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(ZMM30)), operand2: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 126, 202, 230, 54], OperandSize::Qword)
}

