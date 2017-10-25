use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn test_1() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 202], OperandSize::Word)
}

#[test]
fn test_2() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(DI, 4264, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 149, 168, 16], OperandSize::Word)
}

#[test]
fn test_3() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 209], OperandSize::Dword)
}

#[test]
fn test_4() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(EBX, 2082792544, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 139, 96, 228, 36, 124], OperandSize::Dword)
}

#[test]
fn test_5() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 201], OperandSize::Qword)
}

#[test]
fn test_6() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexed(RSI, RDX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 12, 214], OperandSize::Qword)
}

#[test]
fn test_7() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 202], OperandSize::Qword)
}

#[test]
fn test_8() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 1028562445, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 156, 129, 13, 158, 78, 61], OperandSize::Qword)
}

#[test]
fn test_9() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(SI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 222], OperandSize::Word)
}

#[test]
fn test_10() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(BX, 27, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 111, 27], OperandSize::Word)
}

#[test]
fn test_11() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(CX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 209], OperandSize::Dword)
}

#[test]
fn test_12() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 48], OperandSize::Dword)
}

#[test]
fn test_13() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(SI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 254], OperandSize::Qword)
}

#[test]
fn test_14() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 44, 211], OperandSize::Qword)
}

#[test]
fn test_15() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 218], OperandSize::Word)
}

#[test]
fn test_16() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(BX, 136, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 191, 136, 0], OperandSize::Word)
}

#[test]
fn test_17() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 251], OperandSize::Dword)
}

#[test]
fn test_18() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 1950471533, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 140, 177, 109, 213, 65, 116], OperandSize::Dword)
}

#[test]
fn test_19() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 226], OperandSize::Qword)
}

#[test]
fn test_20() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(RAX, 1111135933, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 176, 189, 150, 58, 66], OperandSize::Qword)
}

#[test]
fn test_21() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(RBX)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 133, 235], OperandSize::Qword)
}

#[test]
fn test_22() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(RCX, 1193420836, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 133, 161, 36, 40, 34, 71], OperandSize::Qword)
}

#[test]
fn test_23() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AL)), operand2: Some(Literal8(21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[168, 21], OperandSize::Word)
}

#[test]
fn test_24() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AL)), operand2: Some(Literal8(72)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[168, 72], OperandSize::Dword)
}

#[test]
fn test_25() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AL)), operand2: Some(Literal8(26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[168, 26], OperandSize::Qword)
}

#[test]
fn test_26() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AX)), operand2: Some(Literal16(20111)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[169, 143, 78], OperandSize::Word)
}

#[test]
fn test_27() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AX)), operand2: Some(Literal16(17675)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 169, 11, 69], OperandSize::Dword)
}

#[test]
fn test_28() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AX)), operand2: Some(Literal16(31070)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 169, 94, 121], OperandSize::Qword)
}

#[test]
fn test_29() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EAX)), operand2: Some(Literal32(178110783)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 169, 63, 193, 157, 10], OperandSize::Word)
}

#[test]
fn test_30() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1581551632)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[169, 16, 144, 68, 94], OperandSize::Dword)
}

#[test]
fn test_31() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EAX)), operand2: Some(Literal32(216362663)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[169, 167, 110, 229, 12], OperandSize::Qword)
}

#[test]
fn test_32() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(RAX)), operand2: Some(Literal32(380420291)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 169, 195, 192, 172, 22], OperandSize::Qword)
}

#[test]
fn test_33() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(CL)), operand2: Some(Literal8(14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 193, 14], OperandSize::Word)
}

#[test]
fn test_34() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(BP, 12765, Some(OperandSize::Byte), None)), operand2: Some(Literal8(0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 134, 221, 49, 0], OperandSize::Word)
}

#[test]
fn test_35() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(DL)), operand2: Some(Literal8(115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 194, 115], OperandSize::Dword)
}

#[test]
fn test_36() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 1560755285, Some(OperandSize::Byte), None)), operand2: Some(Literal8(16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 132, 191, 85, 60, 7, 93, 16], OperandSize::Dword)
}

#[test]
fn test_37() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(DL)), operand2: Some(Literal8(101)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 194, 101], OperandSize::Qword)
}

#[test]
fn test_38() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(78)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 1, 78], OperandSize::Qword)
}

#[test]
fn test_39() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(DL)), operand2: Some(Literal8(37)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 194, 37], OperandSize::Qword)
}

#[test]
fn test_40() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 824213468, Some(OperandSize::Byte), None)), operand2: Some(Literal8(112)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 4, 197, 220, 127, 32, 49, 112], OperandSize::Qword)
}

#[test]
fn test_41() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(DI)), operand2: Some(Literal16(24472)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 199, 152, 95], OperandSize::Word)
}

#[test]
fn test_42() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Indirect(BX, Some(OperandSize::Word), None)), operand2: Some(Literal16(10938)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 7, 186, 42], OperandSize::Word)
}

#[test]
fn test_43() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(SI)), operand2: Some(Literal16(4457)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 198, 105, 17], OperandSize::Dword)
}

#[test]
fn test_44() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand2: Some(Literal16(7746)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 2, 66, 30], OperandSize::Dword)
}

#[test]
fn test_45() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(SP)), operand2: Some(Literal16(20254)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 196, 30, 79], OperandSize::Qword)
}

#[test]
fn test_46() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledDisplaced(RCX, Two, 1318352022, Some(OperandSize::Word), None)), operand2: Some(Literal16(7870)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 4, 77, 150, 116, 148, 78, 190, 30], OperandSize::Qword)
}

#[test]
fn test_47() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(ESI)), operand2: Some(Literal32(157752062)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 198, 254, 26, 103, 9], OperandSize::Word)
}

#[test]
fn test_48() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(BX, 76, Some(OperandSize::Dword), None)), operand2: Some(Literal32(127188360)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 71, 76, 136, 189, 148, 7], OperandSize::Word)
}

#[test]
fn test_49() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EDX)), operand2: Some(Literal32(759525736)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 194, 104, 113, 69, 45], OperandSize::Dword)
}

#[test]
fn test_50() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledDisplaced(ESI, Four, 35613117, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1156728834)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 4, 181, 189, 105, 31, 2, 2, 72, 242, 68], OperandSize::Dword)
}

#[test]
fn test_51() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EDI)), operand2: Some(Literal32(361004023)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 199, 247, 123, 132, 21], OperandSize::Qword)
}

#[test]
fn test_52() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Eight, 948458361, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1156132165)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 132, 255, 121, 83, 136, 56, 69, 45, 233, 68], OperandSize::Qword)
}

#[test]
fn test_53() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(RSI)), operand2: Some(Literal32(1670526669)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 198, 205, 54, 146, 99], OperandSize::Qword)
}

#[test]
fn test_54() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 653964079, Some(OperandSize::Qword), None)), operand2: Some(Literal32(384215115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 132, 241, 47, 179, 250, 38, 75, 168, 230, 22], OperandSize::Qword)
}

