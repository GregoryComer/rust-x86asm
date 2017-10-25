use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpaddw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 253, 211], OperandSize::Dword)
}

fn vpaddw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1242625574, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 253, 28, 141, 38, 246, 16, 74], OperandSize::Dword)
}

fn vpaddw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 253, 199], OperandSize::Qword)
}

fn vpaddw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RBX, 1378362763, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 253, 187, 139, 37, 40, 82], OperandSize::Qword)
}

fn vpaddw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 253, 252], OperandSize::Dword)
}

fn vpaddw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EAX, 1518702124, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 253, 144, 44, 142, 133, 90], OperandSize::Dword)
}

fn vpaddw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 253, 248], OperandSize::Qword)
}

fn vpaddw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Two, 837845324, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 253, 164, 112, 76, 129, 240, 49], OperandSize::Qword)
}

fn vpaddw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 139, 253, 221], OperandSize::Dword)
}

fn vpaddw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EAX, 1432031427, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 140, 253, 128, 195, 16, 91, 85], OperandSize::Dword)
}

fn vpaddw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 125, 132, 253, 244], OperandSize::Qword)
}

fn vpaddw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 21, 134, 253, 52, 186], OperandSize::Qword)
}

fn vpaddw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 171, 253, 207], OperandSize::Dword)
}

fn vpaddw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 530987337, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 173, 253, 28, 253, 73, 57, 166, 31], OperandSize::Dword)
}

fn vpaddw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM30)), operand3: Some(Direct(YMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 1, 13, 161, 253, 212], OperandSize::Qword)
}

fn vpaddw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 85, 173, 253, 34], OperandSize::Qword)
}

