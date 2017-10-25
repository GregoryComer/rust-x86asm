use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovsxbq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 34, 221], OperandSize::Dword)
}

fn vpmovsxbq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1434144833, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 34, 36, 221, 65, 80, 123, 85], OperandSize::Dword)
}

fn vpmovsxbq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 34, 195], OperandSize::Qword)
}

fn vpmovsxbq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 24258293, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 34, 12, 141, 245, 38, 114, 1], OperandSize::Qword)
}

fn vpmovsxbq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 34, 216], OperandSize::Dword)
}

fn vpmovsxbq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(ESI, ECX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 34, 12, 142], OperandSize::Dword)
}

fn vpmovsxbq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 34, 196], OperandSize::Qword)
}

fn vpmovsxbq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 34, 44, 94], OperandSize::Qword)
}

fn vpmovsxbq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 34, 203], OperandSize::Dword)
}

fn vpmovsxbq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(ECX, EDI, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 34, 20, 121], OperandSize::Dword)
}

fn vpmovsxbq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 66, 125, 137, 34, 192], OperandSize::Qword)
}

fn vpmovsxbq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 34, 60, 90], OperandSize::Qword)
}

fn vpmovsxbq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 34, 206], OperandSize::Dword)
}

fn vpmovsxbq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 34, 0], OperandSize::Dword)
}

fn vpmovsxbq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM17)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 125, 173, 34, 202], OperandSize::Qword)
}

fn vpmovsxbq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 897075926, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 34, 52, 221, 214, 74, 120, 53], OperandSize::Qword)
}

fn vpmovsxbq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 34, 226], OperandSize::Dword)
}

fn vpmovsxbq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectDisplaced(EDX, 1335241533, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 34, 146, 61, 43, 150, 79], OperandSize::Dword)
}

fn vpmovsxbq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 34, 125, 201, 34, 218], OperandSize::Qword)
}

fn vpmovsxbq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(ZMM18)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1641966596, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 125, 206, 34, 20, 189, 4, 108, 222, 97], OperandSize::Qword)
}

