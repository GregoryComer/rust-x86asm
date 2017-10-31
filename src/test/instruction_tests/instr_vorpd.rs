use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vorpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 86, 238], OperandSize::Dword)
}

#[test]
fn vorpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 86, 59], OperandSize::Dword)
}

#[test]
fn vorpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 86, 213], OperandSize::Qword)
}

#[test]
fn vorpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 86, 32], OperandSize::Qword)
}

#[test]
fn vorpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 86, 246], OperandSize::Dword)
}

#[test]
fn vorpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 86, 50], OperandSize::Dword)
}

#[test]
fn vorpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 86, 197], OperandSize::Qword)
}

#[test]
fn vorpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 246571573, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 86, 20, 253, 53, 98, 178, 14], OperandSize::Qword)
}

#[test]
fn vorpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 229, 142, 86, 198], OperandSize::Dword)
}

#[test]
fn vorpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 213, 139, 86, 36, 136], OperandSize::Dword)
}

#[test]
fn vorpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 436868792, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 253, 159, 86, 132, 153, 184, 22, 10, 26], OperandSize::Dword)
}

#[test]
fn vorpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 161, 197, 138, 86, 206], OperandSize::Qword)
}

#[test]
fn vorpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM15)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 133, 141, 86, 19], OperandSize::Qword)
}

#[test]
fn vorpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 253, 149, 86, 4, 254], OperandSize::Qword)
}

#[test]
fn vorpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 245, 174, 86, 231], OperandSize::Dword)
}

#[test]
fn vorpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1766642156, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 205, 173, 86, 28, 77, 236, 209, 76, 105], OperandSize::Dword)
}

#[test]
fn vorpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1639096813, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 253, 187, 86, 60, 197, 237, 161, 178, 97], OperandSize::Dword)
}

#[test]
fn vorpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 129, 221, 165, 86, 255], OperandSize::Qword)
}

#[test]
fn vorpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Four, 1507583140, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 181, 166, 86, 132, 178, 164, 228, 219, 89], OperandSize::Qword)
}

#[test]
fn vorpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RAX, 1070829393, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 253, 189, 86, 128, 81, 143, 211, 63], OperandSize::Qword)
}

#[test]
fn vorpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 197, 207, 86, 211], OperandSize::Dword)
}

#[test]
fn vorpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Four, 2063787361, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 213, 206, 86, 148, 176, 97, 229, 2, 123], OperandSize::Dword)
}

#[test]
fn vorpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 213, 217, 86, 27], OperandSize::Dword)
}

#[test]
fn vorpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 17, 253, 207, 86, 205], OperandSize::Qword)
}

#[test]
fn vorpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM29)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 149, 197, 86, 32], OperandSize::Qword)
}

#[test]
fn vorpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM22)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 205, 215, 86, 7], OperandSize::Qword)
}

