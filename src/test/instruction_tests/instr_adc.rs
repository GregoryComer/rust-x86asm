use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn adc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 217], OperandSize::Word)
}

#[test]
fn adc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(SI, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 20], OperandSize::Word)
}

#[test]
fn adc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 210], OperandSize::Dword)
}

#[test]
fn adc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 12, 91], OperandSize::Dword)
}

#[test]
fn adc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 211], OperandSize::Qword)
}

#[test]
fn adc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 293763099, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 140, 191, 27, 120, 130, 17], OperandSize::Qword)
}

#[test]
fn adc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 219], OperandSize::Qword)
}

#[test]
fn adc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(RBX, RAX, Four, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 12, 131], OperandSize::Qword)
}

#[test]
fn adc_9() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(SP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 252], OperandSize::Word)
}

#[test]
fn adc_10() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(BX, 35, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 95, 35], OperandSize::Word)
}

#[test]
fn adc_11() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 241], OperandSize::Dword)
}

#[test]
fn adc_12() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(ESI, ESI, Four, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 12, 182], OperandSize::Dword)
}

#[test]
fn adc_13() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 207], OperandSize::Qword)
}

#[test]
fn adc_14() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 38], OperandSize::Qword)
}

#[test]
fn adc_15() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 246], OperandSize::Word)
}

#[test]
fn adc_16() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(SI, 168, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 180, 168, 0], OperandSize::Word)
}

#[test]
fn adc_17() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 243], OperandSize::Dword)
}

#[test]
fn adc_18() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledDisplaced(EBX, Two, 1826125644, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 12, 93, 76, 119, 216, 108], OperandSize::Dword)
}

#[test]
fn adc_19() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 228], OperandSize::Qword)
}

#[test]
fn adc_20() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 36, 206], OperandSize::Qword)
}

#[test]
fn adc_21() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RBP)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 17, 221], OperandSize::Qword)
}

#[test]
fn adc_22() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(RCX, RDI, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 17, 20, 249], OperandSize::Qword)
}

#[test]
fn adc_23() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 201], OperandSize::Word)
}

#[test]
fn adc_24() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[18, 24], OperandSize::Word)
}

#[test]
fn adc_25() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 217], OperandSize::Dword)
}

#[test]
fn adc_26() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 855217648, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[18, 28, 213, 240, 149, 249, 50], OperandSize::Dword)
}

#[test]
fn adc_27() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 217], OperandSize::Qword)
}

#[test]
fn adc_28() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CL)), operand2: Some(IndirectDisplaced(RBX, 2070037194, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[18, 139, 202, 66, 98, 123], OperandSize::Qword)
}

#[test]
fn adc_29() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 217], OperandSize::Qword)
}

#[test]
fn adc_30() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 41635775, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[18, 28, 141, 191, 79, 123, 2], OperandSize::Qword)
}

#[test]
fn adc_31() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 217], OperandSize::Word)
}

#[test]
fn adc_32() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(SI, 16094, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[19, 148, 222, 62], OperandSize::Word)
}

#[test]
fn adc_33() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 210], OperandSize::Dword)
}

#[test]
fn adc_34() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1451204876, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 19, 52, 245, 12, 161, 127, 86], OperandSize::Dword)
}

#[test]
fn adc_35() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 202], OperandSize::Qword)
}

#[test]
fn adc_36() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 19, 28, 151], OperandSize::Qword)
}

#[test]
fn adc_37() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 236], OperandSize::Word)
}

#[test]
fn adc_38() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 6048, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 19, 178, 160, 23], OperandSize::Word)
}

#[test]
fn adc_39() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 212], OperandSize::Dword)
}

#[test]
fn adc_40() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 965892530, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[19, 180, 64, 178, 89, 146, 57], OperandSize::Dword)
}

#[test]
fn adc_41() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 242], OperandSize::Qword)
}

#[test]
fn adc_42() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1517050260, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[19, 44, 141, 148, 89, 108, 90], OperandSize::Qword)
}

#[test]
fn adc_43() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RSP)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 17, 228], OperandSize::Qword)
}

#[test]
fn adc_44() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 19, 44, 134], OperandSize::Qword)
}

#[test]
fn adc_45() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AL)), operand2: Some(Literal8(123)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[20, 123], OperandSize::Word)
}

#[test]
fn adc_46() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AL)), operand2: Some(Literal8(81)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[20, 81], OperandSize::Dword)
}

#[test]
fn adc_47() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[20, 1], OperandSize::Qword)
}

#[test]
fn adc_48() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AX)), operand2: Some(Literal16(31899)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[21, 155, 124], OperandSize::Word)
}

#[test]
fn adc_49() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AX)), operand2: Some(Literal16(14429)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 21, 93, 56], OperandSize::Dword)
}

#[test]
fn adc_50() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AX)), operand2: Some(Literal16(20873)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 21, 137, 81], OperandSize::Qword)
}

#[test]
fn adc_51() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1014519826)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 21, 18, 88, 120, 60], OperandSize::Word)
}

#[test]
fn adc_52() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EAX)), operand2: Some(Literal32(970311115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[21, 203, 197, 213, 57], OperandSize::Dword)
}

#[test]
fn adc_53() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1317302317)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[21, 45, 112, 132, 78], OperandSize::Qword)
}

#[test]
fn adc_54() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RAX)), operand2: Some(Literal32(1980347790)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 21, 142, 181, 9, 118], OperandSize::Qword)
}

#[test]
fn adc_55() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BL)), operand2: Some(Literal8(87)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 211, 87], OperandSize::Word)
}

#[test]
fn adc_56() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(SI, 6, Some(OperandSize::Byte), None)), operand2: Some(Literal8(18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 84, 6, 18], OperandSize::Word)
}

#[test]
fn adc_57() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 210, 90], OperandSize::Dword)
}

#[test]
fn adc_58() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(ESI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(103)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 22, 103], OperandSize::Dword)
}

#[test]
fn adc_59() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(Literal8(39)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 210, 39], OperandSize::Qword)
}

#[test]
fn adc_60() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledDisplaced(RSI, Four, 914740424, Some(OperandSize::Byte), None)), operand2: Some(Literal8(26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 20, 181, 200, 212, 133, 54, 26], OperandSize::Qword)
}

#[test]
fn adc_61() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BL)), operand2: Some(Literal8(33)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 211, 33], OperandSize::Qword)
}

#[test]
fn adc_62() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Four, 1689083061, Some(OperandSize::Byte), None)), operand2: Some(Literal8(47)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 148, 150, 181, 92, 173, 100, 47], OperandSize::Qword)
}

#[test]
fn adc_63() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DI)), operand2: Some(Literal16(21918)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 215, 158, 85], OperandSize::Word)
}

#[test]
fn adc_64() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 88, Some(OperandSize::Word), None)), operand2: Some(Literal16(22593)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 80, 88, 65, 88], OperandSize::Word)
}

#[test]
fn adc_65() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(SI)), operand2: Some(Literal16(27900)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 214, 252, 108], OperandSize::Dword)
}

#[test]
fn adc_66() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand2: Some(Literal16(28717)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 18, 45, 112], OperandSize::Dword)
}

#[test]
fn adc_67() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BP)), operand2: Some(Literal16(12391)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 213, 103, 48], OperandSize::Qword)
}

#[test]
fn adc_68() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledDisplaced(RCX, Four, 589764965, Some(OperandSize::Word), None)), operand2: Some(Literal16(5825)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 20, 141, 101, 25, 39, 35, 193, 22], OperandSize::Qword)
}

#[test]
fn adc_69() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ESP)), operand2: Some(Literal32(1839302384)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 212, 240, 134, 161, 109], OperandSize::Word)
}

#[test]
fn adc_70() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal32(881388863)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 19, 63, 237, 136, 52], OperandSize::Word)
}

#[test]
fn adc_71() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ESP)), operand2: Some(Literal32(144051108)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 212, 164, 11, 150, 8], OperandSize::Dword)
}

#[test]
fn adc_72() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1258479762)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 18, 146, 224, 2, 75], OperandSize::Dword)
}

#[test]
fn adc_73() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ECX)), operand2: Some(Literal32(927425870)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 209, 78, 101, 71, 55], OperandSize::Qword)
}

#[test]
fn adc_74() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Four, 2068429040, Some(OperandSize::Dword), None)), operand2: Some(Literal32(447013808)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 148, 184, 240, 184, 73, 123, 176, 227, 164, 26], OperandSize::Qword)
}

#[test]
fn adc_75() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RBP)), operand2: Some(Literal32(48675558)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 213, 230, 186, 230, 2], OperandSize::Qword)
}

#[test]
fn adc_76() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand2: Some(Literal32(416690633)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 22, 201, 49, 214, 24], OperandSize::Qword)
}

#[test]
fn adc_77() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(SI)), operand2: Some(Literal8(123)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 214, 123], OperandSize::Word)
}

#[test]
fn adc_78() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(117)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 19, 117], OperandSize::Word)
}

#[test]
fn adc_79() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BP)), operand2: Some(Literal8(27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 213, 27], OperandSize::Dword)
}

#[test]
fn adc_80() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand2: Some(Literal8(117)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 19, 117], OperandSize::Dword)
}

#[test]
fn adc_81() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BP)), operand2: Some(Literal8(3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 213, 3], OperandSize::Qword)
}

#[test]
fn adc_82() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Word), None)), operand2: Some(Literal8(35)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 20, 153, 35], OperandSize::Qword)
}

#[test]
fn adc_83() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EBX)), operand2: Some(Literal8(57)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 211, 57], OperandSize::Word)
}

#[test]
fn adc_84() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Memory(1002, Some(OperandSize::Dword), None)), operand2: Some(Literal8(13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 22, 234, 3, 13], OperandSize::Word)
}

#[test]
fn adc_85() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ECX)), operand2: Some(Literal8(62)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 209, 62], OperandSize::Dword)
}

#[test]
fn adc_86() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledDisplaced(ESI, Four, 1605091147, Some(OperandSize::Dword), None)), operand2: Some(Literal8(25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 20, 181, 75, 191, 171, 95, 25], OperandSize::Dword)
}

#[test]
fn adc_87() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ESP)), operand2: Some(Literal8(32)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 212, 32], OperandSize::Qword)
}

#[test]
fn adc_88() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 264539094, Some(OperandSize::Dword), None)), operand2: Some(Literal8(57)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 20, 253, 214, 139, 196, 15, 57], OperandSize::Qword)
}

#[test]
fn adc_89() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RSI)), operand2: Some(Literal8(95)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 214, 95], OperandSize::Qword)
}

#[test]
fn adc_90() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Qword), None)), operand2: Some(Literal8(64)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 20, 182, 64], OperandSize::Qword)
}

