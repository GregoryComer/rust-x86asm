use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vandnpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 85, 215], OperandSize::Dword)
}

fn vandnpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EBX, 181656331, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 85, 187, 11, 219, 211, 10], OperandSize::Dword)
}

fn vandnpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 85, 234], OperandSize::Qword)
}

fn vandnpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 85, 14], OperandSize::Qword)
}

fn vandnpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 85, 249], OperandSize::Dword)
}

fn vandnpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 85, 20, 159], OperandSize::Dword)
}

fn vandnpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 85, 247], OperandSize::Qword)
}

fn vandnpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1009822375, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 85, 4, 197, 167, 170, 48, 60], OperandSize::Qword)
}

fn vandnpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 141, 85, 213], OperandSize::Dword)
}

fn vandnpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 205, 142, 85, 4, 203], OperandSize::Dword)
}

fn vandnpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 197, 155, 85, 60, 136], OperandSize::Dword)
}

fn vandnpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 81, 197, 138, 85, 225], OperandSize::Qword)
}

fn vandnpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM9)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 181, 142, 85, 26], OperandSize::Qword)
}

fn vandnpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1486454753, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 245, 159, 85, 20, 117, 225, 127, 153, 88], OperandSize::Qword)
}

fn vandnpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 245, 173, 85, 239], OperandSize::Dword)
}

fn vandnpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1812566844, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 213, 169, 85, 20, 213, 60, 147, 9, 108], OperandSize::Dword)
}

fn vandnpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Eight, 909421856, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 253, 188, 85, 188, 216, 32, 173, 52, 54], OperandSize::Dword)
}

fn vandnpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM30)), operand3: Some(Direct(YMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 161, 141, 162, 85, 252], OperandSize::Qword)
}

fn vandnpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 424618135, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 245, 173, 85, 60, 133, 151, 40, 79, 25], OperandSize::Qword)
}

fn vandnpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 253, 181, 85, 12, 112], OperandSize::Qword)
}

fn vandnpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 205, 204, 85, 197], OperandSize::Dword)
}

fn vandnpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(ESI, 85554154, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 207, 85, 134, 234, 115, 25, 5], OperandSize::Dword)
}

fn vandnpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 205, 217, 85, 26], OperandSize::Dword)
}

fn vandnpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM23)), operand3: Some(Direct(ZMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 65, 197, 197, 85, 250], OperandSize::Qword)
}

fn vandnpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 1862751408, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 173, 198, 85, 172, 246, 176, 84, 7, 111], OperandSize::Qword)
}

fn vandnpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 197, 218, 85, 42], OperandSize::Qword)
}

